use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m4 4 2.5 2.5" ></ path > < path d = "M13.5 6.5a4.95 4.95 0 0 0-7 7" ></ path > < path d = "M15 5 5 15" ></ path > < path d = "M14 17v.01" ></ path > < path d = "M10 16v.01" ></ path > < path d = "M13 13v.01" ></ path > < path d = "M16 10v.01" ></ path > < path d = "M11 20v.01" ></ path > < path d = "M17 14v.01" ></ path > < path d = "M20 11v.01" ></ path > < / > } } pub const LUCIDE_SHOWER_HEAD : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2")] } ;