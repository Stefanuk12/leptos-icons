use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v2" ></ path > < path d = "m4.93 4.93 1.41 1.41" ></ path > < path d = "M20 12h2" ></ path > < path d = "m19.07 4.93-1.41 1.41" ></ path > < path d = "M15.947 12.65a4 4 0 0 0-5.925-4.128" ></ path > < path d = "M3 20a5 5 0 1 1 8.9-4H13a3 3 0 0 1 2 5.24" ></ path > < path d = "M11 20v2" ></ path > < path d = "M7 19v2" ></ path > < / > } } pub const LUCIDE_CLOUD_SUN_RAIN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24")] } ;