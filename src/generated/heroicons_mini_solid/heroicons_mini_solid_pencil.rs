use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m2.695 14.762-1.262 3.155a.5.5 0 0 0 .65.65l3.155-1.262a4 4 0 0 0 1.343-.886L17.5 5.501a2.121 2.121 0 0 0-3-3L3.58 13.419a4 4 0 0 0-.885 1.343Z" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_PENCIL : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "currentColor") , ("aria-hidden" , "true") , ("viewBox" , "0 0 20 20")] } ;