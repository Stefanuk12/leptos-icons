use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 4H3" ></ path > < path d = "M18 8H6" ></ path > < path d = "M19 12H9" ></ path > < path d = "M16 16h-6" ></ path > < path d = "M11 20H9" ></ path > < / > } } pub const LUCIDE_TORNADO : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;