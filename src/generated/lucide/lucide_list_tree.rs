use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 12h-8" ></ path > < path d = "M21 6H8" ></ path > < path d = "M21 18h-8" ></ path > < path d = "M3 6v4c0 1.1.9 2 2 2h3" ></ path > < path d = "M3 10v6c0 1.1.9 2 2 2h3" ></ path > < / > } } pub const LUCIDE_LIST_TREE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24")] } ;