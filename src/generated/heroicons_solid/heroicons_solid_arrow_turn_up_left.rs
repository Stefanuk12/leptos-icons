use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M20.239 20.25a.75.75 0 0 1-.75-.75V8.999H5.549l2.47 2.47a.75.75 0 0 1-1.06 1.06l-3.75-3.75a.75.75 0 0 1 0-1.06l3.75-3.75a.75.75 0 1 1 1.06 1.06l-2.47 2.47h14.69a.75.75 0 0 1 .75.75V19.5a.75.75 0 0 1-.75.75Z" clip - rule = "evenodd" fill - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_TURN_UP_LEFT : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("aria-hidden" , "true") , ("fill" , "currentColor")] } ;