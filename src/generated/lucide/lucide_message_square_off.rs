use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 15V5a2 2 0 0 0-2-2H9" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M3.6 3.6c-.4.3-.6.8-.6 1.4v16l4-4h10" ></ path > < / > } } pub const LUCIDE_MESSAGE_SQUARE_OFF : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor")] } ;