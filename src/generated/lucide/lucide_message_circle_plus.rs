use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" ></ path > < path d = "M8 12h8" ></ path > < path d = "M12 8v8" ></ path > < / > } } pub const LUCIDE_MESSAGE_CIRCLE_PLUS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;