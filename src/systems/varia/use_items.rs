use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesSensing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[read_component(Equipment)]
#[read_component(Equiped)]
#[read_component(Weapon)]
#[read_component(ProjectileStack)]
#[read_component(Player)]
#[read_component(Name)]
#[read_component(Armour)]
#[write_component(FieldOfView)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    let mut fov = <&mut FieldOfView>::query().filter(component::<Player>());
    let mut player_is_sensing = false;
    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }

                if let Ok(sensing) = item.get_component::<ProvidesSensing>() {
                    player_is_sensing = true;
                }

                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }

                if let Ok(_equipment) = item.get_component::<Equipment>() {
                    commands.remove_component::<Carried>(activate.item);
                    commands.add_component(activate.item, Equiped(activate.used_by));

                    if let Ok(e) = ecs.entry_ref(activate.item) {
                        if e.get_component::<Weapon>().is_ok() {
                            <(Entity, &Equiped, &Weapon)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == activate.used_by)
                                .for_each(|(e, _, _)| {
                                    commands.remove(*e);
                                })
                        }
                        if e.get_component::<Armour>().is_ok() {
                            <(Entity, &Equiped, &Armour)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == activate.used_by)
                                .for_each(|(e, _, _)| {
                                    commands.remove(*e);
                                })
                        }
                        if e.get_component::<ProjectileStack>().is_ok() {
                            let mut stack_amount = e.get_component::<ProjectileStack>().unwrap().0;

                            <(Entity, &Equiped, &ProjectileStack, &Name)>::query()
                                .iter(ecs)
                                .filter(|(_, equiped, _projectile, _)| {
                                    equiped.0 == activate.used_by
                                })
                                .for_each(|(entity, _, projectile, name)| {
                                    let e_name = e.get_component::<Name>().unwrap();
                                    if e_name.0 == name.0 {
                                        stack_amount += projectile.0;
                                        commands.remove(*entity);
                                    }
                                });
                            commands.add_component(activate.item, ProjectileStack(stack_amount));
                        }
                    }
                } else {
                    commands.remove(activate.item);
                }
            }
            commands.remove(*entity);
        });

    for heal in &healing_to_apply {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }

    if player_is_sensing {
        fov.iter_mut(ecs).next().unwrap().sensing = true;
    }
}
