use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4.25 2A2.25 2.25 0 0 0 2 4.25v2a.75.75 0 0 0 1.5 0v-2a.75.75 0 0 1 .75-.75h2a.75.75 0 0 0 0-1.5h-2ZM13.75 2a.75.75 0 0 0 0 1.5h2a.75.75 0 0 1 .75.75v2a.75.75 0 0 0 1.5 0v-2A2.25 2.25 0 0 0 15.75 2h-2ZM3.5 13.75a.75.75 0 0 0-1.5 0v2A2.25 2.25 0 0 0 4.25 18h2a.75.75 0 0 0 0-1.5h-2a.75.75 0 0 1-.75-.75v-2ZM18 13.75a.75.75 0 0 0-1.5 0v2a.75.75 0 0 1-.75.75h-2a.75.75 0 0 0 0 1.5h2A2.25 2.25 0 0 0 18 15.75v-2ZM7 10a3 3 0 1 1 6 0 3 3 0 0 1-6 0Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_VIEWFINDER_CIRCLE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 20 20") , ("data-slot" , "icon") , ("fill" , "currentColor")] } ;