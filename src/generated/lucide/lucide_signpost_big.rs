use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9H4L2 7l2-2h6" ></ path > < path d = "M14 5h6l2 2-2 2h-6" ></ path > < path d = "M10 22V4a2 2 0 1 1 4 0v18" ></ path > < path d = "M8 22h8" ></ path > < / > } } pub const LUCIDE_SIGNPOST_BIG : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;