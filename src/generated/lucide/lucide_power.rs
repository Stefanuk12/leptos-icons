use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v10" ></ path > < path d = "M18.4 6.6a9 9 0 1 1-12.77.04" ></ path > < / > } } pub const LUCIDE_POWER : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24")] } ;