use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2.246 3.743a.75.75 0 0 1 .75.75v6.75h9v-6.75a.75.75 0 0 1 1.5 0v15.002a.75.75 0 1 1-1.5 0v-6.751h-9v6.75a.75.75 0 1 1-1.5 0v-15a.75.75 0 0 1 .75-.75ZM18.75 10.5c-.727 0-1.441.054-2.138.16a.75.75 0 1 1-.223-1.484 15.867 15.867 0 0 1 3.635-.125c1.149.092 2.153.923 2.348 2.115.084.516.128 1.045.128 1.584 0 1.065-.676 1.927-1.531 2.354l-2.89 1.445a1.5 1.5 0 0 0-.829 1.342v.86h4.5a.75.75 0 1 1 0 1.5H16.5a.75.75 0 0 1-.75-.75v-1.61a3 3 0 0 1 1.659-2.684l2.89-1.445c.447-.223.701-.62.701-1.012a8.32 8.32 0 0 0-.108-1.342c-.075-.457-.47-.82-.987-.862a14.45 14.45 0 0 0-1.155-.046Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_H_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("fill" , "currentColor") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon")] } ;