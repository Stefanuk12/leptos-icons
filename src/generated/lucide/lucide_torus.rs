use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse cx = "12" ry = "2" rx = "3" cy = "11" ></ ellipse > < ellipse cx = "12" cy = "12.5" ry = "8.5" rx = "10" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;