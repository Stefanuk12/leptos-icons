use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < ellipse rx = "3" cx = "12" cy = "11" ry = "2" ></ ellipse > < ellipse cy = "12.5" cx = "12" ry = "8.5" rx = "10" ></ ellipse > < / > } } pub const LUCIDE_TORUS : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;