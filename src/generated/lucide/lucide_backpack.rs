use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 10a4 4 0 0 1 4-4h8a4 4 0 0 1 4 4v10a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2Z" ></ path > < path d = "M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" ></ path > < path d = "M8 21v-5a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v5" ></ path > < path d = "M8 10h8" ></ path > < path d = "M8 18h8" ></ path > < / > } } pub const LUCIDE_BACKPACK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("height" , "24")] } ;