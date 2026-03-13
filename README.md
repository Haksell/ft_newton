# ft_newton

## mandatory

- [x] a simple `make` command should be enough to compile the full project
- [ ] a scene where you use either a catapult or a trebuchet throwing some apples (projectiles) on some unstable composed structures
- [ ] impact must produce a series of reaction that is physically correct and the system must go back to a stable state (no infinite bounce)
- you will code at least 3 primitive colliders based on different geometries:
  - [ ] box collider
  - [ ] sphere collider
  - [ ] plane collider
- [ ] the game play can be 2d only (like the real angry bird), but your physic and rendering engine must be in 3d
- [ ] it must be easy to spawn a lot of physical objects on demand
- [ ] an fps counter must be present
- [ ] an object counter must be present
- [ ] the scene should be playable with a normal user display without any debugging information
- a full and direct control of at least:
  - [ ] time (reversal is not mandatory)
  - [ ] gravity
  - [ ] projectile speed
  - [ ] projectile direction
  - [ ] projectile mass
- [ ] a debug display that prints the colliders as wireframes is mandatory and can be turned on/off at any time
- [ ] everything must be considered as a rigid body (no deformation)
- [ ] you will implement the translational motion
- [ ] you will implement rotational motion (it's harder, but you don't need a perfect version here)
- [ ] you will implement elastic collisions
- [ ] you will implement gravity

## bonus

- [ ] good ragdoll
- [ ] a mesh collider that can create an accurate collider based on any 3d model. you need to prepare at least 3 models including 1 humanoid and be prepared to show the precision of the collider with specific testing
- [ ] nice gui
- better primitives (more than box/sphere/plane)
  - [ ] capsule collider
  - [ ] cylinder collider
  - [ ] cone collider
  - [ ] convex hull collider
- [ ] advanced physics (???)
- [ ] softbody physics
- [ ] ft_maxwell (electromagnetism)
- [ ] ft_einstein (relativity)
- [ ] use rotors instead of quaternions (using `ultraviolet`?)
