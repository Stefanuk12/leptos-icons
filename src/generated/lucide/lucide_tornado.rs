use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 4H3" ></ path > < path d = "M18 8H6" ></ path > < path d = "M19 12H9" ></ path > < path d = "M16 16h-6" ></ path > < path d = "M11 20H9" ></ path > < / > } } pub const LUCIDE_TORNADO : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;