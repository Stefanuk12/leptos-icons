use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 7V5a2 2 0 0 1 2-2h2" ></ path > < path d = "M17 3h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M21 17v2a2 2 0 0 1-2 2h-2" ></ path > < path d = "M7 21H5a2 2 0 0 1-2-2v-2" ></ path > < rect y = "8" height = "8" rx = "1" x = "7" width = "10" ></ rect > < / > } } pub const LUCIDE_FULLSCREEN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;