use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 1.5a.75.75 0 0 1 .75.75V7.5h-1.5V2.25A.75.75 0 0 1 12 1.5ZM11.25 7.5v5.69l-1.72-1.72a.75.75 0 0 0-1.06 1.06l3 3a.75.75 0 0 0 1.06 0l3-3a.75.75 0 1 0-1.06-1.06l-1.72 1.72V7.5h3.75a3 3 0 0 1 3 3v9a3 3 0 0 1-3 3h-9a3 3 0 0 1-3-3v-9a3 3 0 0 1 3-3h3.75Z" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_DOWN_ON_SQUARE : Path = Path { path : icon_path , props : & [("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("aria-hidden" , "true") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;