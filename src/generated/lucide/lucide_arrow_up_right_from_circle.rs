use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 12A10 10 0 1 1 12 2" ></ path > < path d = "M22 2 12 12" ></ path > < path d = "M16 2h6v6" ></ path > < / > } } pub const LUCIDE_ARROW_UP_RIGHT_FROM_CIRCLE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;