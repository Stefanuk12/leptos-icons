use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.24 3.75a.75.75 0 0 1-.75.75H8.989v13.939l2.47-2.47a.75.75 0 1 1 1.06 1.061l-3.75 3.75a.75.75 0 0 1-1.06 0l-3.751-3.75a.75.75 0 1 1 1.06-1.06l2.47 2.469V3.75a.75.75 0 0 1 .75-.75H19.49a.75.75 0 0 1 .75.75Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_TURN_LEFT_DOWN : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;