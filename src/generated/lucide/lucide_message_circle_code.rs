use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9.5 8 12l2 2.5" ></ path > < path d = "m14 9.5 2 2.5-2 2.5" ></ path > < path d = "M7.9 20A9 9 0 1 0 4 16.1L2 22z" ></ path > < / > } } pub const LUCIDE_MESSAGE_CIRCLE_CODE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("fill" , "none")] } ;