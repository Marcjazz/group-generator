
# Rust Program for Group Exercise

This Rust program is aimed to automatically assign students to groups, each associated with a topic, while ensuring each student falls under a group which has a topic. 
The groups and their details are saved after the run of the program, which are only accessible by the program in a read-only format.

## Requirements

- **Student registration**: Students must be registered within the system.
- **Shuffling**: Students are shuffled randomly to ensure fairness in group assignment.
  
- **Topics**:
  
  - Each group is assigned a unique topic.
  - The number of topics cannot exceed the number of students.
  - Topics form groups
  - Topics are assigned to a difficulty level of either easy, medium or hard.
    

- **Group setup**:

  - Each group must have a label.
  - Students should be evenly distributed across the groups based on topic difficulty.
  - A student cannot belong to more than one group.
      
- **File Management**:
  
  - A read-only file accessed only from the app after saved from previous run.
  
## Key Features

1. **Balanced group**:
    - Groups will have an even distribution of students based on the total number of students available.
    - No student will be assigned to more than one group.

2. **Topic**:
    - Each group will be assigned a unique topic. The number of topics should not exceed the number of students, ensuring that every group has a dedicated topic.


## Objectives

- **Group File Creation**: every group be in the  unique file with its label and associated students.
- **File Security**: ensure the file are read-only and can only be accessed by the program.
- **Balanced Group Distribution**: distribute students evenly across all groups based on class size.
  
### Optional

- **File storage in binary**: For more efficient storage and retrieval of group data, the files may be saved in a binary format.
### Additional features
 **Colors**
   - Added colors for more readibility and styling 
  
  
## Usage
### Option I
- Clone this repository and run using cargo 
```bash
git clone https://github.com/Marcjazz/group-generator.git
```
- cd into the directory and run **cargo run** given you have rust install in your machine.
- If not check out the install process and enviroment activation of the [officail Rust Website](https://www.rust-lang.org/tools/install)

```bash
cargo run
```
## Accessing the saved application state after execution
- After the successful run of the application you can access the last saved state by entering the **ID** of the saved app state on the next run of the app.

### Option II
- clone the repository, cd into it and build the image using docker
  
```bash
   docker build -t group .
```
- Run the image with a docker container with a volume to the container this allows the program to save it's data from previous run on the host machine and can access the saved state data after every run

```bash
docker run -it -v $(pwd)/data:/usr/src/group-generator/data group
```
