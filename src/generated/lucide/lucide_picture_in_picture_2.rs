use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4" ></ path > < rect x = "12" y = "13" height = "7" width = "10" rx = "2" ></ rect > < / > } } pub const LUCIDE_PICTURE_IN_PICTURE_2 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round")] } ;