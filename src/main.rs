use leptos::prelude::*;
use js_sys::Date;

#[component]
fn App() -> impl IntoView {

    // Boolean signal to determine if Task entry form is opened
    let (task_form, task_form_writer) = signal(false);

    // A Signal to hold a vector of Tasks
    let (tasks, tasks_writer) = signal(Vec::<Task>::new());

    // A signal to store the selected Task from the list. Initial value is None as nothing is selected.
    let (selected_task_id, selected_task_id_writer) = signal(Option::<u32>::None);

    // The get_tasks() function is used to simulate a fetch request for a list of Tasks
    // Call to this function will most likely be in a spawn function.
    for task in get_tasks() {
        tasks_writer.write().push(task);
    }

    // A signal to store a counter that is incremeneted and assigned to new Tasks as the Tasks unique id. 
    // Initial value is the length of the tasks returned from get_tasks()
    // In a fullstack app, the counter would not be required since we would store the Task in a database and return a unique Task id
    let (counter, counter_writer) = signal(get_tasks().len() as u32);

    view! {
        <main>

            // Renders the add Task button
            <ActionBar 
                task_form_writer=task_form_writer
                selected_task_id_writer=selected_task_id_writer />

            // Renders the tasks store in the tasks singal
            <TaskList 
                tasks=tasks 
                task_form_writer=task_form_writer 
                selected_task_id=selected_task_id
                selected_task_id_writer=selected_task_id_writer
            />

            // Renders the Task details if a task has been selected
            <Show when=move || { selected_task_id.get().is_some() }>
                { move || { 
                    
                    let task_id = selected_task_id.get().unwrap();

                    view! { 
                        <TaskDetails
                            selected_task_id=task_id
                        /> 
                    }
                }}
            </Show>

            // Renders the Task entry form if task_form is true
            // task_form value is set in the ActionBar component
            <Show when=move || { task_form.get() }>
                <TaskForm
                    task_form_writer=task_form_writer 
                    counter=counter
                    counter_writer=counter_writer
                    tasks_writer=tasks_writer
                />
            </Show>
        </main>
    }
}

#[component]
fn ActionBar(
    task_form_writer : WriteSignal<bool>, 
    selected_task_id_writer : WriteSignal<Option<u32>>) -> impl IntoView {

    view! {
        <div class="action-bar">
            <button on:click=move |_| {
                task_form_writer.set(true);
                selected_task_id_writer.set(None);
            }>
                <svg xmlns="http://www.w3.org/2000/svg" height="40" viewBox="0 -960 960 960" width="40"><path d="M200-200h43.923l427.923-427.923-43.923-43.923L200-243.923V-200Zm-40 40v-100.769l527.231-527.77q6.146-5.481 13.573-8.471 7.427-2.99 15.486-2.99 8.06 0 15.616 2.538 7.556 2.539 13.94 9.154l42.693 42.923q6.615 6.385 9.038 14.008Q800-723.754 800-716.131q0 8.131-2.741 15.558-2.74 7.427-8.72 13.573l-527.77 527H160Zm600.769-556.308-44.461-44.461 44.461 44.461Zm-111.27 66.809-21.576-22.347 43.923 43.923-22.347-21.576Z"></path></svg>
            </button>
        </div>
    }
}

#[component]
fn TaskList(
    tasks : ReadSignal<Vec<Task>>, 
    task_form_writer : WriteSignal<bool>, 
    selected_task_id : ReadSignal<Option<u32>>,
    selected_task_id_writer : WriteSignal<Option<u32>>) -> impl IntoView {

    view! {
        <div class="task-list">
            {move || {
                tasks.get().into_iter().map(|task :Task| {

                    let due_date = task.due_date;

                    let cloned_task1 = task.clone();
                    
                    // Determine if the task is selected and apply css to highlight the row.
                    let item_css = {
                        if selected_task_id.get() == Some(task.id) {
                            "item selected-item"
                        }else{
                            "item"
                        }
                    };

                    let priority = match task.priority {
                        TaskPriority::Low() => "cell task-priority task-priority-low",
                        TaskPriority::Medium() => "cell task-priority task-priority-medium",
                        TaskPriority::High() => "cell task-priority task-priority-high",
                    };

                    let (status, status_class) = match task.status {
                        TaskStatus::New() => ("new", "task-attribute task-status-new"),
                        TaskStatus::Pending() => ("pending", "task-attribute task-status-pending"),
                        TaskStatus::InProgress() => ("in progress", "task-attribute task-status-inprogress"),
                        TaskStatus::Complete() => ("Complete", "task-attribute task-status-complete"),
                    };

                    view! {
                        <div class={item_css} on:click= move |_| {
                            // When the row is selected, the task's id is stored and the TaskDetails component is rendered 
                            selected_task_id_writer.set(Some(task.id));
                            // In case the TaskForm component is already rendered, we want to remove it so that the TaskDetails component can be rendered. 
                            // TaskForm and TaskDetails components should not be rendered at the same time. 
                            task_form_writer.set(false);
                        }>
                            <h4 class="cell">{task.id}</h4>
                            <i class={priority}></i>
                            <h3 class="cell">{task.name}</h3>
                            <AssignedUsers task=cloned_task1 />
                            <span class="cell width-100-px">{format_date(due_date)}</span>
                            <div class={status_class}><span>{status}</span></div>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}

#[component]
fn TaskDetails(selected_task_id : u32) -> impl IntoView {

    // The get_task() function simulates a fetch request for a single Task using the selected task id.
    // Call to this function will most likely be in a spawn function.
    let task = get_task(selected_task_id); 

    // Clone the task so that the clone can be passed to the AssignedUsers component 
    let clone_task = task.clone();  

    let (priority, priority_class) = match task.priority {
        TaskPriority::Low() => ("Low", "task-attribute task-priority-low"),
        TaskPriority::Medium() => ("Medium", "task-attribute task-priority-medium"),
        TaskPriority::High() => ("High", "task-attribute task-priority-high"),
    };

    let (status, status_class) = match task.status {
        TaskStatus::New() => ("new", "task-attribute task-status-new"),
        TaskStatus::Pending() => ("pending", "task-attribute task-status-pending"),
        TaskStatus::InProgress() => ("in progress", "task-attribute task-status-inprogress"),
        TaskStatus::Complete() => ("Complete", "task-attribute task-status-complete"),
    };

    view! {
        <div class="form-container">
            <div class="row">
                <h2>{task.name}</h2>
                <footer>
                    <ul>
                        <li>
                            <div class={priority_class}>
                                <label>Priority</label>
                                <span>{priority}</span>
                            </div>
                        </li>
                        <li>
                            <div class={status_class}>
                                <label>Status</label>
                                <span>{status}</span>
                            </div>
                        </li>
                        <li>
                            <div class="task-attribute">
                                <label>Due Date</label>
                                <span>{format_date(task.due_date)}</span>
                            </div>
                        </li>
                    </ul>
                </footer>
            </div>

            <div class="row">
                <p>{task.description}</p>
            </div>

            <div class="row">
                <AssignedUsers task=clone_task />
            </div>

            <div>
                <ul class="comment-header">
                    <li>Comments</li>
                </ul>
                {
                    task.comments.into_iter().map(|comment| {
                        view! {
                            <div class="comment">
                                <div class="img">
                                    <img src={format!("static/{}", comment.image)} />
                                </div>
                                <div class="body">
                                    <div><h2>{comment.user}</h2></div>
                                    <div>{comment.message}</div>
                                </div>
                            </div>
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}

#[component]
fn TaskForm(
    task_form_writer : WriteSignal<bool>, 
    counter : ReadSignal<u32>, 
    counter_writer : WriteSignal<u32>, 
    tasks_writer : WriteSignal<Vec<Task>>,) -> impl IntoView {

    let (task, task_writer) = signal(Task::default());

    view! {
        <div class="form-container">
            <div class="input-row">
                <input type="text" placeholder="Task name" on:input = move |e| { 
                    task_writer.write().name = event_target_value(&e)
                } />
            </div>

            <div class="input-row">
                <textarea rows="10" placeholder="Task description" on:input = move |e| { 
                    task_writer.write().description = event_target_value(&e)
                }></textarea>
            </div>

            <AssignUserList
                task=task
                task_writer=task_writer
            />

            <div class="button-container">
                <button on:click = move |_| {

                    counter_writer.set(counter.get() + 1);

                    task_writer.write().id = counter.get();
                    task_writer.write().due_date = Date::now();

                    tasks_writer.write().push(task.get());

                    task_form_writer.set(false);
                }>
                    "Create Task"
                </button>
            </div>
        </div>
    }
}

#[component]
fn AssignedUsers(task : Task) -> impl IntoView {
    view! {
        <ul class="cell task-users-list">
            {
                task.assigned_to.into_iter().map(|u: User| {
                    view! {
                        <li><img src={format!("static/{}", u.image)} /></li>
                    }
                }).collect_view()
            }
        </ul>
    }
}

#[component]
fn AssignUserList(task : ReadSignal<Task>, task_writer : WriteSignal<Task>) -> impl IntoView {

    let (users_to_assign, users_to_assign_writer) = signal(Vec::<AssignUser>::new());

    for user in get_users(){
        users_to_assign_writer.write().push(AssignUser { user: user, is_assigned: false });
    }

    view! {
        <details class="user-list">
            <summary>Assign Users</summary>
            {move || {

                users_to_assign.get().into_iter().enumerate().map(|(index, assign_user)| {

                    let assign_user_clone = assign_user.clone();

                    let mut item_css = "user-item";

                    if assign_user.is_assigned {
                        item_css = "user-item user-item-selected";
                    }

                    view! {
                        <div class={item_css} on:click=move|_| {
                            
                            let mut updated_task = task.get();

                            if updated_task.assigned_to.contains(&assign_user.user) == false {

                                updated_task.assigned_to.push(User { name: assign_user.user.name.clone(), image: assign_user.user.image.clone() });

                                task_writer.set(updated_task); 

                                if let Some(assign_user) = users_to_assign_writer.write().get_mut(index){
                                    assign_user.is_assigned = true;
                                }
                            }
                        }>
                            <img src={format!("static/{}", assign_user_clone.user.image)} />
                            <span>{assign_user_clone.user.name}</span>
                        </div>
                    }
                    
                }).collect_view()
            }}
        </details>
    }
}

fn main() {
    mount_to_body(App);
}

// Function to simulate a fetch request and return a list of Users
fn get_users() -> Vec<User> {
    vec![
        User {
            name : "Derik".to_string(),
            image : "person1.png".to_string(),
        },
        User {
            name : "Fatima".to_string(),
            image : "person2.png".to_string(),
        },
        User {
            name : "John".to_string(),
            image : "person3.png".to_string(),
        },
        User {
            name : "Ilyana".to_string(),
            image : "person4.png".to_string(),
        },
    ]
}

// Function to simulate a fetch request and return a list of Tasks
fn get_tasks() -> Vec<Task> {
    vec![
        Task {
            id : 1,
            name : "Design login screen".to_string(),
            description : "Create a responsive login screen with email and password fields, 'Forgot Password' link, and a login button. Include basic validation and error handling.".to_string(),
            due_date : Date::new_0().add_days(5),
            assigned_to : vec![ 
                User{ name : "Derik".to_string(), image : "person1.png".to_string() }
            ],
            priority : TaskPriority::Medium(),
            status : TaskStatus::Pending(),
            comments : vec![]
        },
        Task {
            id : 2,
            name : "Write unit tests for task API".to_string(),
            description : "Create unit tests for the task-related API endpoints, including task creation, status updates, and deletion. Use mock data and ensure edge cases are covered.".to_string(),
            due_date : Date::new_0().add_days(25),
            assigned_to : vec![ 
                User{ name : "Derik".to_string(), image : "person1.png".to_string() },
                User{ name : "Fatima".to_string(), image : "person2.png".to_string() },
                User{ name : "John".to_string(), image : "person3.png".to_string() },
            ],
            priority : TaskPriority::High(),
            status : TaskStatus::InProgress(),
            comments : vec![]
        },
        Task {
            id : 3,
            name : "Implement product search with filters".to_string(),
            description : "Develop a product search feature that allows users to search by name, category, and price range. Include filter options such as 'In Stock', 'On Sale', and 'Free Shipping'. 
                Ensure the results update dynamically as filters are applied.".to_string(),
            due_date : Date::new_0().add_days(45),
            assigned_to : vec![ 
                User{ name : "Derik".to_string(), image : "person1.png".to_string() },
                User{ name : "Ilyana".to_string(), image : "person4.png".to_string() }
            ],
            priority : TaskPriority::Low(),
            status : TaskStatus::New(),
            comments : vec![]
        },
        Task {
            id : 4,
            name : "Integrate payment gateway".to_string(),
            description : "Set up and integrate a payment gateway (e.g., Stripe or PayPal) to handle secure transactions during checkout. Implement payment validation, error handling, 
                and confirmation messaging. Ensure the system can handle both test and live environments.".to_string(),
            due_date : Date::new_0().add_days(10),
            assigned_to : vec![ 
                User{ name : "Derik".to_string(), image : "person1.png".to_string() },
            ],
            priority : TaskPriority::Medium(),
            status : TaskStatus::Complete(),
            comments : vec![]
        },
        Task {
            id : 5,
            name : "Create order history page".to_string(),
            description : "Build a user-facing order history page that displays past purchases with order details, statuses, and tracking information. Include pagination and filtering by date or status.".to_string(),
            due_date : Date::now(),
            assigned_to : vec![ 
                User{ name : "Ilyana".to_string(), image : "person4.png".to_string() },
            ],
            priority : TaskPriority::High(),
            status : TaskStatus::Pending(),
            comments : vec![]
        },
        Task {
            id : 6,
            name : "Implement product review system".to_string(),
            description : "Allow users to leave reviews and ratings on products. Design the UI for submitting and displaying reviews, and create backend endpoints to store and fetch review data. 
                Include moderation capabilities to filter inappropriate content.".to_string(),
            due_date : Date::new_0().add_days(12),
            assigned_to : vec![ 
                User{ name : "John".to_string(), image : "person3.png".to_string() },
                User{ name : "Ilyana".to_string(), image : "person4.png".to_string() }
            ],
            priority : TaskPriority::Low(),
            status : TaskStatus::InProgress(),
            comments : vec![
                Comment{
                    user : "John".to_string(),
                    message : "Should reviews be visible to everyone immediately, or only after moderation?".to_string(),
                    image: "person3.png".to_string()
                },Comment{
                    user : "Ilyana".to_string(),
                    message : "Good question, I'll get back to you".to_string(),
                    image: "person4.png".to_string()
                },
                Comment{
                    user : "John".to_string(),
                    message : "Ok, thanks, I'll wait for your reply".to_string(),
                    image: "person3.png".to_string()
                }
            ]
        },
        Task {
            id : 7,
            name : "Add wishlist functionality".to_string(),
            description : "Enable users to add products to a personal wishlist for future reference. Implement the UI for adding/removing items and a wishlist page to view saved products. 
                Ensure the wishlist is saved per user and persists across sessions.".to_string(),
            due_date : Date::new_0().add_days(45),
            assigned_to : vec![ 
                User{ name : "Derik".to_string(), image : "person2.png".to_string() }
            ],
            priority : TaskPriority::High(),
            status : TaskStatus::InProgress(),
            comments : vec![]
        }
    ]
}

fn get_task(task_id : u32) -> Task {

    let tasks = get_tasks();

    let task = tasks.iter().find(|task| {
        task.id == task_id
    }).unwrap().clone();

    task
}

fn format_date(timestamp_ms : f64) -> String{

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    let date = Date::new(&timestamp_ms.into());

    let year = date.get_full_year();
    let month = date.get_month();
    let day = date.get_date();

    format!("{} {} {}", day, months[month as usize], year)
}

#[derive(Clone, Default)]
struct Task {
    id : u32,
    name : String,
    description : String,
    assigned_to : Vec<User>,
    due_date : f64,
    priority : TaskPriority,
    status : TaskStatus,
    comments : Vec<Comment>
}
#[derive(Clone)]
struct Comment {
    user : String,
    message : String,
    image : String
}

#[derive(Clone)]
enum TaskStatus {
    New(),
    Pending(),
    InProgress(),
    Complete(),
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::New()
    }
}

#[derive(Clone)]
enum TaskPriority {
    Low(),
    Medium(),
    High()
}

impl Default for TaskPriority {
    fn default() -> Self {
        Self::Low()
    }
}

#[derive(Clone, Default, PartialEq)]
struct User {
    name : String,
    image : String
}

#[derive(Clone, Default, PartialEq)]
struct AssignUser {
    user : User,
    is_assigned : bool
}

pub trait AddDaysExt {
    fn add_days(&self, days : i32) -> f64;
}

impl AddDaysExt for Date {
    fn add_days(&self, days : i32) -> f64 {
        let ms_in_day = 86_400_000.0;
        let new_time = self.get_time() + (days as f64 * ms_in_day);
        new_time
    }
}