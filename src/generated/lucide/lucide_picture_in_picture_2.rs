use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 9V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v10c0 1.1.9 2 2 2h4" ></ path > < rect height = "7" x = "12" y = "13" width = "10" rx = "2" ></ rect > < / > } } pub const LUCIDE_PICTURE_IN_PICTURE_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;