use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16.85 18.58a9 9 0 1 0-9.7 0" ></ path > < path d = "M8 14a5 5 0 1 1 8 0" ></ path > < circle cy = "11" r = "1" cx = "12" ></ circle > < path d = "M13 17a1 1 0 1 0-2 0l.5 4.5a.5.5 0 1 0 1 0Z" ></ path > < / > } } pub const LUCIDE_PODCAST : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;