use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 6v12" ></ path > < path d = "M17.196 9 6.804 15" ></ path > < path d = "m6.804 9 10.392 6" ></ path > < / > } } pub const LUCIDE_ASTERISK : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2")] } ;