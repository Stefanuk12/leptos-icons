use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7.9 20A9 9 0 1 0 4 16.1L2 22Z" ></ path > < path d = "m10 10-2 2 2 2" ></ path > < path d = "m14 10 2 2-2 2" ></ path > < / > } } pub const LUCIDE_MESSAGE_CIRCLE_CODE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none")] } ;