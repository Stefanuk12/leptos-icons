use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.738 3.75c0 .414.336.75.75.75H14.99v13.939l-2.47-2.47a.75.75 0 0 0-1.06 1.061l3.75 3.75a.75.75 0 0 0 1.06 0l3.751-3.75a.75.75 0 0 0-1.06-1.06l-2.47 2.469V3.75a.75.75 0 0 0-.75-.75H4.487a.75.75 0 0 0-.75.75Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_TURN_RIGHT_DOWN : Path = Path { path : icon_path , props : & [("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("fill" , "currentColor")] } ;