use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 2h6l3 7H6l3-7Z" ></ path > < path d = "M12 9v13" ></ path > < path d = "M9 22h6" ></ path > < / > } } pub const LUCIDE_LAMP_FLOOR : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;