use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3.75 3a.75.75 0 0 1 .75.75v7.5h10.94l-1.97-1.97a.75.75 0 0 1 1.06-1.06l3.25 3.25a.75.75 0 0 1 0 1.06l-3.25 3.25a.75.75 0 1 1-1.06-1.06l1.97-1.97H3.75A.75.75 0 0 1 3 12V3.75A.75.75 0 0 1 3.75 3Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_ARROW_TURN_DOWN_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 20 20") , ("aria-hidden" , "true") , ("data-slot" , "icon") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;