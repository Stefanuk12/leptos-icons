use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 6L12 2L16 6" ></ path > < path d = "M12 2V22" ></ path > < / > } } pub const LUCIDE_MOVE_UP : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;