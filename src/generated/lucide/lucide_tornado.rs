use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 4H3" ></ path > < path d = "M18 8H6" ></ path > < path d = "M19 12H9" ></ path > < path d = "M16 16h-6" ></ path > < path d = "M11 20H9" ></ path > < / > } } pub const LUCIDE_TORNADO : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;