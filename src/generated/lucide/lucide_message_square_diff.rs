use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5 19-2 2V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2" ></ path > < path d = "M9 10h6" ></ path > < path d = "M12 7v6" ></ path > < path d = "M9 17h6" ></ path > < / > } } pub const LUCIDE_MESSAGE_SQUARE_DIFF : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;