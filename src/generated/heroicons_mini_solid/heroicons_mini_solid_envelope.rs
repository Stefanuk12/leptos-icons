use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 4a2 2 0 0 0-2 2v1.161l8.441 4.221a1.25 1.25 0 0 0 1.118 0L19 7.162V6a2 2 0 0 0-2-2H3Z" ></ path > < path d = "m19 8.839-7.77 3.885a2.75 2.75 0 0 1-2.46 0L1 8.839V14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V8.839Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ENVELOPE : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 20 20")] } ;