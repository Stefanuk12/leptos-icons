use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path fill - rule = "evenodd" d = "M20.239 3.749a.75.75 0 0 0-.75.75V15H5.549l2.47-2.47a.75.75 0 0 0-1.06-1.06l-3.75 3.75a.75.75 0 0 0 0 1.06l3.75 3.75a.75.75 0 1 0 1.06-1.06L5.55 16.5h14.69a.75.75 0 0 0 .75-.75V4.5a.75.75 0 0 0-.75-.751Z" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_SOLID_ARROW_TURN_DOWN_LEFT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("data-slot" , "icon") , ("viewBox" , "0 0 24 24") , ("fill" , "currentColor") , ("aria-hidden" , "true")] } ;