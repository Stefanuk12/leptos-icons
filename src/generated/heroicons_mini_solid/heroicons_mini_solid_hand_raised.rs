use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 2a1 1 0 1 0-2 0v6.5a.5.5 0 0 1-1 0V3a1 1 0 1 0-2 0v5.5a.5.5 0 0 1-1 0V5a1 1 0 1 0-2 0v7a7 7 0 1 0 14 0V8a1 1 0 1 0-2 0v3.5a.5.5 0 0 1-1 0V3a1 1 0 1 0-2 0v5.5a.5.5 0 0 1-1 0V2Z" fill - rule = "evenodd" clip - rule = "evenodd" ></ path > < / > } } pub const HEROICONS_MINI_SOLID_HAND_RAISED : Path = Path { path : icon_path , props : & [("fill" , "currentColor") , ("data-slot" , "icon") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 20 20") , ("aria-hidden" , "true")] } ;