use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16.85 18.58a9 9 0 1 0-9.7 0" ></ path > < path d = "M8 14a5 5 0 1 1 8 0" ></ path > < circle cx = "12" cy = "11" r = "1" ></ circle > < path d = "M13 17a1 1 0 1 0-2 0l.5 4.5a.5.5 0 1 0 1 0Z" ></ path > < / > } } pub const LUCIDE_PODCAST : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;