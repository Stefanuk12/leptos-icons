use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M3 9h18v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V9Z" ></ path > < path d = "m3 9 2.45-4.9A2 2 0 0 1 7.24 3h9.52a2 2 0 0 1 1.8 1.1L21 9" ></ path > < path d = "M12 3v6" ></ path > < / > } } pub const LUCIDE_PACKAGE_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2")] } ;