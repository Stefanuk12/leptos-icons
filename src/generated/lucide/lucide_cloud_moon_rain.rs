use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10.188 8.5A6 6 0 0 1 16 4a1 1 0 0 0 6 6 6 6 0 0 1-3 5.197" ></ path > < path d = "M11 20v2" ></ path > < path d = "M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" ></ path > < path d = "M7 19v2" ></ path > < / > } } pub const LUCIDE_CLOUD_MOON_RAIN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;