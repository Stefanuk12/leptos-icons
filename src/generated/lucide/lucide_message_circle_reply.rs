use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" ></ path > < path d = "m10 15-3-3 3-3" ></ path > < path d = "M7 12h7a2 2 0 0 1 2 2v1" ></ path > < / > } } pub const LUCIDE_MESSAGE_CIRCLE_REPLY : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;