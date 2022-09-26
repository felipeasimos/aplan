# A plan

All you need is a plan.

`aplan` is a simple tool to manage projects from your terminal.

It is also provided as a crate.

## Roadmap

- [ ] WSB manager
  - [x] Store tasks with hashmap
  - [x] Control change to tasks (add, remove, update)
  - [x] inherent value analysis
  - [x] PV/AC
  - [ ] SPI/PV/EV/SPI/SV/CPI/CV
  - [ ] Member management
  - [ ] Gantt
  - [ ] CPM
- [ ] Burndown Manager
  - [ ] Viewer
  - [ ] Plot
  - [ ] Manage story

### Features

It has 2 main subsystems:

* `WSB` - Arrange the tasks in a tree structure, where children status affects the parent.

* `Burndown` - Arrange the tasks in a priority queue, where tasks have status of "not checked out", "checked out" and "done".

Both can be accessed through a subcommand:

```
aplan wsb show
aplan wsb add 0 "Create WSB"
aplan wsb rm

aplan burndown show
aplan bd show tasks
```

or, with the crate:

```
let mut project = Project::new("aplan.ap");
let task = project.wsb().add_task("1.1", "Value analysis").unwrap();
```
