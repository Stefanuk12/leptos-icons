use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path clip - rule = "evenodd" fill - rule = "evenodd" d = "M3.74 20.25a.75.75 0 0 0 .75-.75V8.999h13.938l-2.47 2.47a.75.75 0 0 0 1.061 1.06l3.75-3.75a.75.75 0 0 0 0-1.06l-3.75-3.75a.75.75 0 0 0-1.06 1.06l2.47 2.47H3.738a.75.75 0 0 0-.75.75V19.5c0 .414.336.75.75.75Z" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_TURN_UP_RIGHT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("aria-hidden" , "true")] } ;