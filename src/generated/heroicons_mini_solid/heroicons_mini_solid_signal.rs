use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16.364 3.636a.75.75 0 0 0-1.06 1.06 7.5 7.5 0 0 1 0 10.607.75.75 0 0 0 1.06 1.061 9 9 0 0 0 0-12.728ZM4.697 4.697a.75.75 0 0 0-1.061-1.061 9 9 0 0 0 0 12.728.75.75 0 1 0 1.06-1.06 7.5 7.5 0 0 1 0-10.607Z" ></ path > < path d = "M12.475 6.464a.75.75 0 0 1 1.06 0 5 5 0 0 1 0 7.072.75.75 0 0 1-1.06-1.061 3.5 3.5 0 0 0 0-4.95.75.75 0 0 1 0-1.06ZM7.525 6.464a.75.75 0 0 1 0 1.061 3.5 3.5 0 0 0 0 4.95.75.75 0 0 1-1.06 1.06 5 5 0 0 1 0-7.07.75.75 0 0 1 1.06 0ZM11 10a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_SIGNAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 20 20") , ("fill" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;