use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 7h2a5 5 0 0 1 0 10h-2m-6 0H7A5 5 0 0 1 7 7h2" ></ path > < / > } } pub const LUCIDE_UNLINK_2 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;