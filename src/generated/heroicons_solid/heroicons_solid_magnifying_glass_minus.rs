use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" clip - rule = "evenodd" d = "M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Zm4.5 0a.75.75 0 0 1 .75-.75h6a.75.75 0 0 1 0 1.5h-6a.75.75 0 0 1-.75-.75Z" ></ path > < / > } } pub const HEROICONS_SOLID_MAGNIFYING_GLASS_MINUS : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true")] } ;