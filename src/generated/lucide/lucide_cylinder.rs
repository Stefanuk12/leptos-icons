use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cx = "12" rx = "9" ry = "3" cy = "5" ></ ellipse > < path d = "M3 5v14a9 3 0 0 0 18 0V5" ></ path > < / > } } pub const LUCIDE_CYLINDER : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;