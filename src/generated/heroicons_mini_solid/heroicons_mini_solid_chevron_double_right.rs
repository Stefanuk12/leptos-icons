use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.28 9.47a.75.75 0 0 1 0 1.06l-4.25 4.25a.75.75 0 1 1-1.06-1.06L13.69 10 9.97 6.28a.75.75 0 0 1 1.06-1.06l4.25 4.25ZM6.03 5.22l4.25 4.25a.75.75 0 0 1 0 1.06l-4.25 4.25a.75.75 0 0 1-1.06-1.06L8.69 10 4.97 6.28a.75.75 0 0 1 1.06-1.06Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_CHEVRON_DOUBLE_RIGHT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "currentColor") , ("viewBox" , "0 0 20 20") , ("aria-hidden" , "true")] } ;