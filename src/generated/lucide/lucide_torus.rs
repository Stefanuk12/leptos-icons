use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse rx = "3" cy = "11" cx = "12" ry = "2" ></ ellipse > < ellipse cx = "12" rx = "10" ry = "8.5" cy = "12.5" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;