use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9 2h6l3 7H6l3-7Z" ></ path > < path d = "M12 9v13" ></ path > < path d = "M9 22h6" ></ path > < / > } } pub const LUCIDE_LAMP_FLOOR : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;