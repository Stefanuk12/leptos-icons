use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" ></ path > < path d = "m10 8-2 2 2 2" ></ path > < path d = "m14 8 2 2-2 2" ></ path > < / > } } pub const LUCIDE_MESSAGE_SQUARE_CODE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;