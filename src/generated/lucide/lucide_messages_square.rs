use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14 9a2 2 0 0 1-2 2H6l-4 4V4c0-1.1.9-2 2-2h8a2 2 0 0 1 2 2z" ></ path > < path d = "M18 9h2a2 2 0 0 1 2 2v11l-4-4h-6a2 2 0 0 1-2-2v-1" ></ path > < / > } } pub const LUCIDE_MESSAGES_SQUARE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round")] } ;