use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path stroke - linecap = "round" stroke - linejoin = "round" d = "M3.75 12h16.5m-16.5 3.75h16.5M3.75 19.5h16.5M5.625 4.5h12.75a1.875 1.875 0 0 1 0 3.75H5.625a1.875 1.875 0 0 1 0-3.75Z" ></ path > < / > } } pub const HEROICONS_OUTLINE_QUEUE_LIST : Path = Path { path : icon_path , props : & [("stroke-width" , "1.5") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("data-slot" , "icon") , ("aria-hidden" , "true") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;