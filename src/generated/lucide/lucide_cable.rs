use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 9a2 2 0 0 1-2-2V5h6v2a2 2 0 0 1-2 2Z" ></ path > < path d = "M3 5V3" ></ path > < path d = "M7 5V3" ></ path > < path d = "M19 15V6.5a3.5 3.5 0 0 0-7 0v11a3.5 3.5 0 0 1-7 0V9" ></ path > < path d = "M17 21v-2" ></ path > < path d = "M21 21v-2" ></ path > < path d = "M22 19h-6v-2a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2Z" ></ path > < / > } } pub const LUCIDE_CABLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor")] } ;