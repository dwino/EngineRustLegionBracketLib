use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(WantsToRangedAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(RangedDamage)]
#[read_component(Defense)]
#[read_component(Carried)]
#[read_component(Equiped)]
#[write_component(ProjectileStack)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut ranged_attackers = <(Entity, &WantsToRangedAttack)>::query();
    let ranged_victims: Vec<(Entity, Entity, Entity)> = ranged_attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();
    ranged_victims
        .iter()
        .for_each(|(message, attacker, victim)| {
            let is_player = ecs
                .entry_ref(*victim)
                .unwrap()
                .get_component::<Player>()
                .is_ok();

            let base_ranged_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
                if let Ok(dmg) = v.get_component::<RangedDamage>() {
                    dmg.0
                } else {
                    0
                }
            } else {
                0
            };

            let base_defense = if let Ok(v) = ecs.entry_ref(*victim) {
                if let Ok(dfn) = v.get_component::<Defense>() {
                    dfn.0
                } else {
                    0
                }
            } else {
                0
            };

            let mut weapon_damage = 0;

            //TODO: check of de attacker de player is (nu check je enkel het victim), en algemeen opkuisen
            if !is_player {
                if let Some((entity, ranged_damage, mut projectile)) =
                    <(Entity, &Equiped, &RangedDamage, &mut ProjectileStack)>::query()
                        .iter_mut(ecs)
                        .filter(|(_, equiped, _, _)| equiped.0 == *attacker)
                        .find_map(|(entity, _, dmg, projectile)| Some((entity, dmg.0, projectile)))
                {
                    weapon_damage = ranged_damage;

                    projectile.0 -= 1;
                    if projectile.0 < 1 {
                        commands.remove(*entity);
                    }
                }
            }

            let armour_defense: i32 = <(&Equiped, &Defense)>::query()
                .iter(ecs)
                .filter(|(carried, _)| carried.0 == *victim)
                .map(|(_, dfn)| dfn.0)
                .sum();

            let final_damage = base_ranged_damage + weapon_damage - base_defense - armour_defense;

            if let Ok(mut health) = ecs
                .entry_mut(*victim)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current -= i32::max(final_damage, 0);
                if health.current < 1 && !is_player {
                    commands.remove(*victim);
                }
            }
            commands.remove(*message);
        });

    let mut melee_attackers = <(Entity, &WantsToAttack)>::query();
    let melee_victims: Vec<(Entity, Entity, Entity)> = melee_attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    melee_victims
        .iter()
        .for_each(|(message, attacker, victim)| {
            let is_player = ecs
                .entry_ref(*victim)
                .unwrap()
                .get_component::<Player>()
                .is_ok();

            let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
                if let Ok(dmg) = v.get_component::<Damage>() {
                    dmg.0
                } else {
                    0
                }
            } else {
                0
            };

            let weapon_damage: i32 = <(&Equiped, &Damage)>::query()
                .iter(ecs)
                .filter(|(carried, _)| carried.0 == *attacker)
                .map(|(_, dmg)| dmg.0)
                .sum();

            let base_defense = if let Ok(v) = ecs.entry_ref(*victim) {
                if let Ok(dfn) = v.get_component::<Defense>() {
                    dfn.0
                } else {
                    0
                }
            } else {
                0
            };

            let armour_defense: i32 = <(&Equiped, &Defense)>::query()
                .iter(ecs)
                .filter(|(carried, _)| carried.0 == *victim)
                .map(|(_, dfn)| dfn.0)
                .sum();

            let final_damage = base_damage + weapon_damage - base_defense - armour_defense;

            if let Ok(mut health) = ecs
                .entry_mut(*victim)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current -= i32::max(final_damage, 0);
                if health.current < 1 && !is_player {
                    commands.remove(*victim);
                }
            }
            commands.remove(*message);
        });
}
